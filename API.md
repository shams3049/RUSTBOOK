# API Documentation

## Authentication

### Register a new user

**Endpoint**: `POST /api/auth/register`

**Request Body**:
```json
{
  "username": "johndoe",
  "email": "john@example.com",
  "password": "securepassword"
}
```

**Response**:
```json
{
  "status": "success",
  "message": "User registered successfully",
  "user_id": "uuid-string"
}
```

### Login

**Endpoint**: `POST /api/auth/login`

**Request Body**:
```json
{
  "email": "john@example.com",
  "password": "securepassword"
}
```

**Response**:
```json
{
  "status": "success",
  "token": "jwt-token-string"
}
```

## Resources

### Get all items

**Endpoint**: `GET /api/items`

**Headers**:
- `Authorization: Bearer jwt-token-string`

**Response**:
```json
{
  "status": "success",
  "data": [
    {
      "id": "uuid-string",
      "name": "Item Name",
      "description": "Item Description",
      "created_at": "2023-01-01T00:00:00Z"
    }
  ]
}
```

### Get item by ID

**Endpoint**: `GET /api/items/{id}`

**Headers**:
- `Authorization: Bearer jwt-token-string`

**Response**:
```json
{
  "status": "success",
  "data": {
    "id": "uuid-string",
    "name": "Item Name",
    "description": "Item Description",
    "created_at": "2023-01-01T00:00:00Z"
  }
}
```

### Create a new item

**Endpoint**: `POST /api/items`

**Headers**:
- `Authorization: Bearer jwt-token-string`

**Request Body**:
```json
{
  "name": "New Item",
  "description": "Item Description"
}
```

**Response**:
```json
{
  "status": "success",
  "message": "Item created successfully",
  "item_id": "uuid-string"
}
```

### Update an item

**Endpoint**: `PUT /api/items/{id}`

**Headers**:
- `Authorization: Bearer jwt-token-string`

**Request Body**:
```json
{
  "name": "Updated Item Name",
  "description": "Updated Item Description"
}
```

**Response**:
```json
{
  "status": "success",
  "message": "Item updated successfully"
}
```

### Delete an item

**Endpoint**: `DELETE /api/items/{id}`

**Headers**:
- `Authorization: Bearer jwt-token-string`

**Response**:
```json
{
  "status": "success",
  "message": "Item deleted successfully"
}
```

## Error Responses

All endpoints may return the following error responses:

**400 Bad Request**:
```json
{
  "status": "error",
  "message": "Invalid input data"
}
```

**401 Unauthorized**:
```json
{
  "status": "error",
  "message": "Authentication required"
}
```

**404 Not Found**:
```json
{
  "status": "error",
  "message": "Resource not found"
}
```

**500 Internal Server Error**:
```json
{
  "status": "error",
  "message": "An internal server error occurred"
}
```
