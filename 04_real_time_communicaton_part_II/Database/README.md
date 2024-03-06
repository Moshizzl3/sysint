# Database Access Documentation

## Overview

This document outlines the access privileges and connection instructions for different users in the database. Privileges are granted on a user level and NOT role level.

### Users and Their Privileges

1. **myreaduserall**
   - **Description**: Read-only access to all tables
   - **Privileges**: `SELECT` on all tables in the `public` schema
   - **Usage**:
    <details>
    <summary>Click to expand</summary>

     ```
     pgcli -h dbservername.postgres.database.azure.com -p 5432 -U myreaduserall testdb
     PASSWORD: '123'
     ```√√√SELECT * FROM users;SELECT * FROM users;

     Example
     ```sql
     SELECT * FROM users;
     ```
     </details>

2. **myreadusersome**
   - **Description**: Read-only access to specific tables
   - **Privileges**: `SELECT` on `accounts`, `transactions` tables
   - **Usage**:

     ```
     pgcli -h dbservername.postgres.database.azure.com -p 5432 -U myreadusersome testdb
     PASSWORD: '123'
     ```

3. **mywriteuserall**
   - **Description**: Full access to all tables
   - **Privileges**: `SELECT`, `INSERT`, `UPDATE`, `DELETE` on all tables in the `public` schema
   - **Usage**:

     ```
     pgcli -h dbservername.postgres.database.azure.com -p 5432 -U mywriteuserall testdb
     PASSWORD: '123'
     ```

4. **mywriteusersome**
   - **Description**: Full access to specific tables
   - **Privileges**: `SELECT`, `INSERT`, `UPDATE`, `DELETE` on `accounts` table
   - **Usage**:

     ```
     pgcli -h dbservername.postgres.database.azure.com -p 5432 -U mywriteusersome testdb
     PASSWORD: '123'
     ```

5. **myviewuser**
   - **Description**: Read-only access to views
   - **Privileges**: `SELECT` on `tranction_view`
   - **Usage**:

     ```
     pgcli -h dbservername.postgres.database.azure.com -p 5432 -U myviewuser testdb
     PASSWORD: '123'
     ```

6. **myfunctionuser**
   - **Description**: Execute specific functions
   - **Privileges**: `EXECUTE` on `generate_test_data(INT, INT, INT)` function
   - **Usage**:

     ```
     pgcli -h dbservername.postgres.database.azure.com -p 5432 -U myfunctionuser testdb
     PASSWORD: '123'
     ```

7. **mycolumnuserread**
   - **Description**: Read-only access to specific columns
   - **Privileges**: `SELECT` (`id`, `user_name`) on `users` table
   - **Usage**:

     ```
     pgcli -h dbservername.postgres.database.azure.com -p 5432 -U mycolumnuserread testdb
     PASSWORD: '123'
     ```
