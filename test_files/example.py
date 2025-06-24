def hello_world():
    print("Hello, World!")

class Person:
    def __init__(self, name):
        self.name = name
    
    def greet(self):
        print(f"Hello, I'm {self.name}")

if __name__ == "__main__":
    hello_world()
    person = Person("Alice")
    person.greet()