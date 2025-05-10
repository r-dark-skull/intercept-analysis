/// <reference path="../pb_data/types.d.ts" />
migrate((app) => {
  const collection = new Collection({
    "createRule": "",
    "deleteRule": null,
    "fields": [
      {
        "autogeneratePattern": "[a-z0-9]{15}",
        "hidden": false,
        "id": "text3208210256",
        "max": 15,
        "min": 15,
        "name": "id",
        "pattern": "^[a-z0-9]+$",
        "presentable": false,
        "primaryKey": true,
        "required": true,
        "system": true,
        "type": "text"
      },
      {
        "autogeneratePattern": "",
        "hidden": false,
        "id": "text1872009285",
        "max": 0,
        "min": 0,
        "name": "time",
        "pattern": "",
        "presentable": false,
        "primaryKey": false,
        "required": false,
        "system": false,
        "type": "text"
      },
      {
        "cascadeDelete": false,
        "collectionId": "pbc_4216348639",
        "hidden": false,
        "id": "relation2177882855",
        "maxSelect": 1,
        "minSelect": 0,
        "name": "url_id",
        "presentable": false,
        "required": false,
        "system": false,
        "type": "relation"
      },
      {
        "hidden": false,
        "id": "json1188578755",
        "maxSize": 0,
        "name": "url_parameters",
        "presentable": false,
        "required": false,
        "system": false,
        "type": "json"
      },
      {
        "hidden": false,
        "id": "number276513331",
        "max": null,
        "min": null,
        "name": "response_status",
        "onlyInt": false,
        "presentable": false,
        "required": false,
        "system": false,
        "type": "number"
      },
      {
        "hidden": false,
        "id": "json3710336466",
        "maxSize": 0,
        "name": "request_headers",
        "presentable": false,
        "required": false,
        "system": false,
        "type": "json"
      },
      {
        "hidden": false,
        "id": "json1555630587",
        "maxSize": 0,
        "name": "response_headers",
        "presentable": false,
        "required": false,
        "system": false,
        "type": "json"
      },
      {
        "convertURLs": false,
        "hidden": false,
        "id": "editor2752457800",
        "maxSize": 0,
        "name": "request_body",
        "presentable": false,
        "required": false,
        "system": false,
        "type": "editor"
      },
      {
        "convertURLs": false,
        "hidden": false,
        "id": "editor1997078824",
        "maxSize": 0,
        "name": "response_body",
        "presentable": false,
        "required": false,
        "system": false,
        "type": "editor"
      },
      {
        "hidden": false,
        "id": "number1778825629",
        "max": null,
        "min": null,
        "name": "response_length",
        "onlyInt": false,
        "presentable": false,
        "required": false,
        "system": false,
        "type": "number"
      },
      {
        "hidden": false,
        "id": "autodate2990389176",
        "name": "created",
        "onCreate": true,
        "onUpdate": false,
        "presentable": false,
        "system": false,
        "type": "autodate"
      },
      {
        "hidden": false,
        "id": "autodate3332085495",
        "name": "updated",
        "onCreate": true,
        "onUpdate": true,
        "presentable": false,
        "system": false,
        "type": "autodate"
      }
    ],
    "id": "pbc_3149572702",
    "indexes": [
      "CREATE UNIQUE INDEX `idx_upXGSRN5Ci` ON `intercepted_requests` (\n  `url_id`,\n  `url_parameters`,\n  `request_body`\n)",
      "CREATE INDEX `idx_GihsctfIzu` ON `intercepted_requests` (\n  `url_id`,\n  `url_parameters`,\n  `response_status`,\n  `request_body`\n)"
    ],
    "listRule": "",
    "name": "intercepted_requests",
    "system": false,
    "type": "base",
    "updateRule": null,
    "viewRule": ""
  });

  return app.save(collection);
}, (app) => {
  const collection = app.findCollectionByNameOrId("pbc_3149572702");

  return app.delete(collection);
})
