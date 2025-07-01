(class_declaration
 name: (identifier) @name.definition.class
 ) @definition.class

(class_declaration
    (base_list (_) @name.reference.class)
 ) @reference.class

(interface_declaration
 name: (identifier) @name.definition.interface
 ) @definition.interface

(interface_declaration
  (base_list (_) @name.reference.interface)
 ) @reference.interface

(method_declaration
 name: (identifier) @name.definition.method
 ) @definition.method

; Constructor declarations (special case of method_declaration)
(constructor_declaration
 name: (identifier) @name.definition.method
 ) @definition.method

; Property declarations
(property_declaration
 name: (identifier) @name.definition.property
 ) @definition.property


(object_creation_expression
  (identifier) @name.reference.class
 ) @reference.class

(type_parameter_constraints_clause
  (identifier) @name.reference.class
 ) @reference.class

;(type_constraint
; type: (identifier) @name.reference.class
; ) @reference.class

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

(namespace_declaration
 name: (identifier) @name.definition.module
) @definition.module