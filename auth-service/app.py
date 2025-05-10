from fastapi import FastAPI, HTTPException, Response, Cookie
from fastapi.responses import HTMLResponse
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import Optional
from pocketbase import Client
import os

app = FastAPI()

# Configure CORS
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Initialize PocketBase client
pb = Client(os.getenv("DB_HOST", "http://pocketbase-service:8090"))


class UserLogin(BaseModel):
    username: str
    password: str


class UserRegister(BaseModel):
    username: str
    password: str
    email: str


@app.get("/ui/login")
async def login_page():
    return HTMLResponse(content="<h1>Login Page</h1>", status_code=200)


@app.post("/api/auth/login")
async def login(user: UserLogin, response: Response):
    try:
        # Authenticate user with PocketBase
        auth_data = pb.collection('users').auth_with_password(
            user.username,
            user.password
        )

        # Set token in cookie
        response.set_cookie(
            key="auth_token",
            value=auth_data.token,
            httponly=True,
            secure=True,
            samesite="strict"
        )

        return {"message": "Login successful", "user_id": auth_data.record.id}
    except Exception as e:
        raise HTTPException(status_code=401, detail="Invalid credentials")


@app.post("/api/auth/register")
async def register(user: UserRegister):
    try:
        # Create user in PocketBase
        user_data = {
            "username": user.username,
            "email": user.email,
            "password": user.password,
            "passwordConfirm": user.password,
        }

        record = pb.collection('users').create(user_data)
        return {"message": "Registration successful", "user_id": record.id}
    except Exception as e:
        raise HTTPException(status_code=400, detail="Registration failed")


@app.get("/auth/validate")
async def validate_token(auth_token: Optional[str] = Cookie(None)):
    if not auth_token:
        raise HTTPException(status_code=401, detail="No token provided")

    try:
        # Verify token with PocketBase
        print(auth_token)
        pb.auth_store.save(auth_token)
        user_record = pb.collection("users").authRefresh()

        response = Response(status_code=200)
        response.headers["X-USER-ID"] = user_record.record.id
        response.headers["X-USER-NAME"] = user_record.record.username
        response.headers["X-USER-EMAIL"] = user_record.record.email
        return response

    except Exception as e:
        raise HTTPException(status_code=401, detail="Invalid token")

if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="0.0.0.0", port=1337)
