# So what are our epics going to be?

Let's look at the e-commerce document.

# Core  
## Users  
We happen to have 3 **types of users**:
1. Distributors
2. Optician
3. Visitor

At a base level, a user ha s the following properties:
1. Display Name
2. Username
3. Password
4. Email
5. Tel. Number (optional)
6. Profile Pic (optional)
7. Introducer (optional)

Visitors have this additional properties:
1. Supported Cities

## Goal
The goal of this system is to let distributors (and visitors on behalf of distributos(?)) to put their **catalog of frames** on the platform. Then, other distributors, opticians and visitors -a.k.a. users- can see their catalogue. The whole model or only its price can only be seen by **permitted** users which are specified in the form of **followers**.

## Discover  
Optic Mart is all about discoverability. Users should be able to **find** -or get **suggestions**- other users and models on the platform. Then, should an available model catch their attention, users should be able to **order** models. Optic Mart MVP isn't going to have an inventory management system built in, so availability is a simple switch toggled by the owner. Also, ordering is just that, there's no online payment or delivery involved. If a model is not available, then the customer can choose to be **notified** when it goes on sale again. Notifications can also be turned on for new models of users' favorite distributors.

## Frames  
Speaking of availability, let's talk about **frames' properties**:
1. Brand Name
2. Model Name
3. Colors (with availability for each color)
4. Sizes (with availability for each size)
5. Price
6. Availability
7. Material (Wood, Aluminium, etc.)
8. Form (grif)
9. Images
10. Case (with image)
11. Additional Description

**Sunglasses** may also include following additional properties:
1. UV
2. Polarized

Aside from this core scenario, there are some other feature packages that can be added to the platform.

# Reporting  
With all this data in hand, various potentially useful reports can be generated and be offered to the users. Some examples are:
## for Distributors/Visitors  
1. Based on Total/Recent Number of Orders (Best Models, Customers, Cities, etc.)
## for Opticians 
1. Based on Total/Recent Number of Orders (Popular Models, Sellers, etc.)

# Visitor Companion
For the road warriors, Optic Mart offers a companion in the form of a **map** on which Opticians can be found. Upon click on an Optician's icon on the map, an **info card** containing general info about them plus their recent purchase history is shown to the user.

# Blog
Distributors and visitors will have a blog section on their profile so they can share new developments in the field interesting to them with other users on the platform. This blog has to provide a title field in addition to a WYSIWYG editor for creating new posts.
