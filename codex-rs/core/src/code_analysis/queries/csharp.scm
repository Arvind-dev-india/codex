; Class declarations
(class_declaration
 name: (identifier) @name.definition.class
 ) @definition.class

(class_declaration
    (base_list (_) @name.reference.class)
 ) @reference.class

; Record declarations (C# 9+)
(record_declaration
 name: (identifier) @name.definition.class
 ) @definition.class

(record_declaration
    (base_list (_) @name.reference.class)
 ) @reference.class

; Interface declarations
(interface_declaration
 name: (identifier) @name.definition.interface
 ) @definition.interface

(interface_declaration
  (base_list (_) @name.reference.interface)
 ) @reference.interface

; Method declarations
(method_declaration
 name: (identifier) @name.definition.method
 ) @definition.method

; Constructor declarations
(constructor_declaration
 name: (identifier) @name.definition.method
 ) @definition.method

; Property declarations
(property_declaration
 name: (identifier) @name.definition.property
 ) @definition.property

; Field declarations
(field_declaration
  (variable_declaration
    (variable_declarator
      (identifier) @name.definition.field)))

; Enum declarations
(enum_declaration
 name: (identifier) @name.definition.enum
 ) @definition.enum

; Enum member declarations
(enum_member_declaration
 name: (identifier) @name.definition.enum_member
 ) @definition.enum_member

; Delegate declarations
(delegate_declaration
 name: (identifier) @name.definition.delegate
 ) @definition.delegate

; Event declarations
(event_declaration
 name: (identifier) @name.definition.event
 ) @definition.event

; Local function declarations
(local_function_statement
 name: (identifier) @name.definition.method
 ) @definition.method

; Lambda expressions
(lambda_expression) @definition.lambda

; Object creation expressions
(object_creation_expression
  (identifier) @name.reference.class
 ) @reference.class

; Implicit object creation (C# 9+) - commented out as may not be supported by current Tree-sitter C# parser
; (implicit_object_creation_expression) @reference.class

; Type parameter constraints
(type_parameter_constraints_clause
  (identifier) @name.reference.class
 ) @reference.class

; Variable declarations with type references
(variable_declaration
  (identifier) @name.reference.class
 ) @reference.class

; Member access method calls (e.g., Console.WriteLine, object.Method)
(invocation_expression
 function:
  (member_access_expression
    name: (identifier) @name.reference.send
 )
) @reference.send

; Simple method calls (e.g., MethodB(), Add())
(invocation_expression
 function: (identifier) @name.reference.call
) @reference.call

; Await expressions
(await_expression
  (invocation_expression
    function: (identifier) @name.reference.call
  )
) @reference.call

; LINQ query expressions
(query_expression) @reference.linq

; Switch expressions (C# 8+)
(switch_expression) @reference.switch

; Pattern matching
(is_pattern_expression) @reference.pattern

; Using declarations - commented out as not supported by current Tree-sitter C# parser
; (using_declaration) @reference.using

; Namespace declarations
(namespace_declaration
 name: (identifier) @name.definition.module
) @definition.module

; File scoped namespace (C# 10+) - commented out as may not be supported by current Tree-sitter C# parser
; (file_scoped_namespace_declaration
;  name: (identifier) @name.definition.module
; ) @definition.module

; Attribute usage
(attribute
  name: (identifier) @name.reference.attribute
) @reference.attribute

; Generic type parameters
(type_parameter
  (identifier) @name.definition.type_parameter
) @definition.type_parameter

; Extension method calls
(invocation_expression
  function: 
    (member_access_expression
      expression: (_)
      name: (identifier) @name.reference.extension_method
    )
) @reference.extension_method