## A quick note for developers working on the database logic.

(Yes, I mean myself)

To store the icalendar datatypes in the database, I had to implement the logic for it.
For this, I chose to go with the DAO pattern (Data Access Object).

### The DAO pattern

A word from phind.com:

The Data Access Object (DAO) pattern is a design pattern that separates the 
data persistence logic from the business logic of an application. 
It provides an abstraction layer over the data source, allowing the application to 
interact with the data without being tightly coupled to the underlying data storage 
mechanism.

## For converting, I chose to go with the Adapter pattern

The Adapter Pattern is a structural design pattern that allows objects with 
incompatible interfaces to work together by converting the interface of one class into 
an interface expected by the clients. It achieves this by wrapping the original class 
with a new interface that matches the client's expectations, thus enabling interoperability
between otherwise incompatible components.
