# Feedback

Setup went good, and i was able to view all tables: 
<br>
<img src="screenshots/setup.png" alt="alt text" width="400"/>

## Users Table


Accessing allowed columns:
<br>
<img src="screenshots/users_success.png" alt="alt text" width="400"/>

Trying to access all columns failed, as expected:
<br>
<img src="screenshots/users_denied.png" alt="alt text" width="400"/>

Trying to update i also got denied:
<br>
<img src="screenshots/users_update_denied.png" alt="alt text" width="400"/>

## Products Table

Accessing all allowed columns, `*` allowed here:
<br>
<img src="screenshots/products_succes.png" alt="alt text" width="400"/>

Updating was allowed on the specific fields:
<br>
<img src="screenshots/products_update_success.png" alt="alt text" width="400"/>

Updating other fields, expected to get denied:
<br>
<img src="screenshots/products_update_denied.png" alt="alt text" width="400"/>

Deleting was also denied as expected:
<br>
<img src="screenshots/products_delete_denied.png" alt="alt text" width="400"/>


## Order table

Selecting all orders:
<br>
<img src="screenshots/order_select_success.png" alt="alt text" width="400"/>

Updating was allowed:
<br>
<img src="screenshots/order_update_success.png" alt="alt text" width="400"/>

Insert was allowed:
<br>
<img src="screenshots/order_insert_success.png" alt="alt text" width="400"/>

Deleting was denied as expected:
<br>
<img src="screenshots/order_delete_denied.png" alt="alt text" width="400"/>

## order_products

Selecting all orders was denied, i should be able to view it?
<br>
<img src="screenshots/order_products_denied.png" alt="alt text" width="400"/>


Updating  was denied, i should be able to update it?
<br>
<img src="screenshots/order_products_denied_update.png" alt="alt text" width="400"/>


Insert was denied, i should be able to insert?
<br>
<img src="screenshots/order_products_denied_insert.png" alt="alt text" width="400"/>

Deleting was denied as expected:
<br>
<img src="screenshots/order_products_denied_delete.png" alt="alt text" width="400"/>

