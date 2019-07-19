# Distributary Model  
A distributary is a firm that has following properties:  
|			|type	|mandatory	|additional constraints							|additional info	|
|:---------------------:|:-----:|:-------------:|:---------------------------------------------------------------------:|:---------------------:|
|id			|number	|[x]		|unsigned; unique among distributors					|			|
|name			|string	|[x]		|minimum 2 and maximum 50 characters					|used as display name	|
|establishmentdate	|date	|		|									|			|
|address		|string |		|maximum 500 characters							|			|
|phonenumber		|string	|		|minimum 4 and maximum 20 characters; only digits, + and - are allowed. |			|
|email			|string	|[x]		|includes @[A-z].[A-z][A-z]						|			|
|username		|string	|[x]		|minimum 2 and maximum 20 characters					|			|
|password		|string	|[x]		|minimum 2 and maximum 50 characters					|			|
|photo			|string	|		|									|in base64 format	|


