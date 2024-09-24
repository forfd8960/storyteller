# API

## Register

```url
POST /v1/register
```

### Request

```json
{
    "email": "",
    "password": ""
}
```

### Response

```json
{
    "msg": "success",
}
```

```json
{
    "msg": "email already exists",
}
```

```json
{
    "msg": "password too weak",
}
```

## Login

```sh
POST /v1/login
```

### Login Request

```json
{
    "email": "",
    "password": ""
}
```

### Login Response

200 OK

```json
{

}
```

## Create Story

after create succcess redirectly to story detail page

```url
POST /v1/stories

{
    "title": "",
    "content": ""
}
```

* Response

```json
{
    "story_id": ""
}
```

## Update Story

after create succcess redirectly to story detail page

```url
PUT /v1/stories/{story_id}

{
    "title": "",
    "content": ""
}
```

* Response

```json
{
    "story_id": ""
}
```

## Get Story - Story Details

```url
GET /v1/stories/{story_id}
```

* Response

```json
{
    "story_id": "",
    "author_id": "",
    "title": "",
    "contnet": "",
    "created_at": "",
    "updated_at": "",
}
```

## Delete Story

```url
DELETE /v1/stories/{story_id}
```

* Response

```json
200 OK
```

## List Stories

* URL: `GET /v1/stories`

* Request

```json
{
    "limit": 0,
    "offset": 100,
    "key_word": "",
    "order_by": "create_at desc" // "updated_at desc"
}
```

* Response

```json
{
    "stories": [
        {
            "story_id": "",
            "author_id": "",
            "title": "",
            "contnet": "",
            "created_at": "",
            "updated_at": "",
        }
    ],
    "has_more": true
}
```