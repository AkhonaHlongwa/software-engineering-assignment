# Reflection on Domain Modeling and Class Diagram Development

## Introduction

This assignment focused on developing a detailed domain model and UML class diagram for the Library Management System created throughout previous assignments. The objective was to identify key entities, define their relationships, model attributes and methods, and represent the structure of the system using Mermaid.js class diagrams.

The assignment built upon previous work completed in requirements analysis, use case modeling, Agile planning, and behavioral modeling. This ensured consistency and alignment throughout the software engineering process.

---

# Challenges Faced During Domain Modeling

One of the major challenges encountered during this assignment was identifying the correct level of abstraction for the domain entities. Initially, it was difficult to determine which entities were essential to the system and which ones would introduce unnecessary complexity.

For example, deciding whether to model FinePayment as a separate entity or simply as part of the Loan entity required careful consideration. Separating it into its own entity improved modularity and responsibility separation, but also increased the number of relationships in the system.

Another challenge involved defining methods and responsibilities for each class. Some methods appeared applicable to multiple entities, which created confusion about ownership and encapsulation. For instance, the calculation of overdue fines could potentially belong to either the Loan class or the FinePayment class. Eventually, the responsibility was assigned to the Loan entity because overdue calculations are directly related to borrowing activity.

Relationship modeling was also challenging. Determining multiplicity relationships such as one-to-many or optional associations required careful analysis of system behavior from previous assignments. For example, one user may borrow many books over time, while one book can only belong to one active loan at a time.

---

# Alignment with Previous Assignments

The class diagram and domain model were strongly aligned with previous assignments completed during the semester.

The functional requirements from Assignment 4 directly influenced the identification of entities and responsibilities. Requirements such as borrowing books, reserving books, processing payments, and managing memberships were translated into corresponding classes and methods.

The use cases from Assignment 5 helped identify interactions between users and system components. Use cases such as “Checkout Book,” “Reserve Book,” and “Pay Fine” guided the creation of relationships between User, Book, Loan, and FinePayment entities.

Behavioral diagrams from Assignment 8 also played a significant role in validating the design. State transition diagrams demonstrated how entities changed state over time, while activity diagrams illustrated workflows and interactions. These diagrams helped confirm that the domain model accurately represented the behavior of the system.

Agile planning concepts from Assignment 6 also influenced the iterative refinement of the design. As new workflows and requirements emerged, the class diagram was adjusted to maintain consistency and scalability.

---

# Trade-Offs and Design Decisions

Several trade-offs were made during the design process.

One important decision involved choosing associations instead of complex inheritance structures. Although inheritance could have been used for entities such as User and Librarian, keeping them as separate classes simplified the overall design and improved readability.

Another trade-off involved balancing detail and simplicity. Adding too many attributes and methods would make the class diagram difficult to read and maintain. Therefore, only the most important business-related attributes and operations were included.

Composition and aggregation relationships were also considered, but many relationships were ultimately represented using associations because the entities could exist independently of one another.

The design also prioritized maintainability and scalability. Entities were structured so that additional features, such as notifications or digital resources, could be added in future iterations without requiring major redesign.

---

# Lessons Learned About Object-Oriented Design

This assignment significantly improved understanding of object-oriented analysis and design principles.

One important lesson learned was the importance of responsibility separation. Each class should represent a clear business concept with focused responsibilities. This improves maintainability and reduces unnecessary coupling between components.

Another lesson involved understanding relationships between entities and how multiplicity affects system behavior. Correctly modeling associations is critical for accurately representing real-world processes.

The assignment also reinforced the importance of abstraction in software engineering. Domain models should focus on important business concepts while avoiding unnecessary implementation details.

Using Mermaid.js to create UML diagrams improved documentation skills and demonstrated how visual models support communication between developers, analysts, and stakeholders.

Finally, the assignment highlighted the importance of aligning structural models with behavioral models, requirements, and use cases to ensure system consistency throughout the software development lifecycle.

---

# Conclusion

Overall, this assignment provided valuable experience in domain modeling and UML class diagram development. It strengthened understanding of object-oriented design principles, system modeling, and software architecture.

The combination of domain analysis, class relationships, and behavioral alignment created a comprehensive representation of the Library Management System and demonstrated how software engineering artifacts work together to support successful system design.
