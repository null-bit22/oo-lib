# Library Checkout System for CSE315
A basic checkout system in Python and Rust, using OOP principles like polymorphism, interhitance, and encapsulation

## FAQ
### **How encapsulation was enforced:**
Generally, object fields were kept private. They are accessed through getters in the Rust module, and using the @property decorator in the Python module

### **Why composition or inheritance:**
For this project, inheritance made more sense. There was only one trait/abstract class that needed to be implemented and that was the Item trait/class. If there were many different kinds of classes/traits that needed to be implemented into a single object, then composition would have been highly considered. 

### **Where polymorphism appears:**
In both the Python and the Rust app, it appears in the 'catalog' source code. Catalog is a dict/HashMap of Items -> you can pass either a Book or a Dvd, the exact type of Item does not matter since they both implement Item. It is easily accomplished in Python by just passing a Book/Dvd (method params suggest 'Item"). In Rust, you can do 
```rust
catalog: HashMap<String, Box<dyn Item>>;
```
And you can pass in either a Book or Dvd since both objects implement the Item trait. This is possible due to dynamic dispatching.

## Run Python App
From project root:
```bash
cd oo-lib-python
```
### Install pytest
```
python3 -m pip install -U pytest
```
### Run
Use `-B` to not create cache directories 
```bash
python3 -B -m app
```
### Run Tests
```bash
python3 -m pytest
```

### Contributors
Patrick Neill
