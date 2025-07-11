; Class definitions
(class_specifier name: (type_identifier) @name.definition.class) @definition.class

; Struct definitions (treated as classes in C++)
(struct_specifier name: (type_identifier) @name.definition.class body:(_)) @definition.class

; Union definitions
(declaration type: (union_specifier name: (type_identifier) @name.definition.class)) @definition.class
(union_specifier name: (type_identifier) @name.definition.class) @definition.class

; Enum definitions
(enum_specifier name: (type_identifier) @name.definition.type) @definition.type

; Type definitions
(type_definition declarator: (type_identifier) @name.definition.type) @definition.type

; Function definitions
(function_definition 
  declarator: (function_declarator 
    declarator: (identifier) @name.definition.function)) @definition.function

; Function declarations
(declaration
  declarator: (function_declarator
    declarator: (identifier) @name.definition.function)) @definition.function

; Function declarations with init_declarator
(declaration
  declarator: (init_declarator
    declarator: (function_declarator
      declarator: (identifier) @name.definition.function))) @definition.function

; Function declarations in parameter lists
(parameter_declaration
  declarator: (function_declarator
    declarator: (identifier) @name.definition.function)) @definition.function

; Function pointer declarations
(declaration
  declarator: (pointer_declarator
    declarator: (function_declarator
      declarator: (identifier) @name.definition.function))) @definition.function

; Method definitions (functions within classes)
(function_definition 
  declarator: (function_declarator 
    declarator: (qualified_identifier 
      scope: (namespace_identifier) @scope 
      name: (identifier) @name.definition.method))) @definition.method

; Constructor definitions
(function_definition 
  declarator: (function_declarator 
    declarator: (qualified_identifier 
      name: (identifier) @name.definition.constructor))) @definition.constructor

; Template function definitions
(template_declaration 
  (function_definition 
    declarator: (function_declarator 
      declarator: (identifier) @name.definition.function))) @definition.function

; Template class definitions
(template_declaration 
  (class_specifier name: (type_identifier) @name.definition.class)) @definition.class

; Namespace definitions
(namespace_definition name: (namespace_identifier) @name.definition.module) @definition.module

; Method calls with member access (obj.method(), obj->method())
(call_expression 
  function: (field_expression 
    field: (field_identifier) @name.reference.call)) @reference.call

; Method calls with qualified names (Class::method())
(call_expression 
  function: (qualified_identifier 
    name: (identifier) @name.reference.call)) @reference.call

; Simple function calls (function())
(call_expression 
  function: (identifier) @name.reference.call) @reference.call

; Constructor calls with new
(new_expression 
  type: (type_identifier) @name.reference.constructor) @reference.constructor

; Type references in variable declarations
(declaration 
  type: (type_identifier) @name.reference.class) @reference.class

; Type references in parameter lists
(parameter_declaration 
  type: (type_identifier) @name.reference.class) @reference.class

; Template instantiations
(template_type 
  name: (type_identifier) @name.reference.class) @reference.class

; Base class references in inheritance
(base_class_clause 
  (type_identifier) @name.reference.class) @reference.class

; Namespace usage
(using_declaration 
  (qualified_identifier 
    scope: (namespace_identifier) @name.reference.module)) @reference.module

; Field access (obj.field, obj->field)
(field_expression 
  field: (field_identifier) @name.reference.field) @reference.field

; Qualified identifiers (namespace::identifier)
(qualified_identifier 
  scope: (namespace_identifier) @name.reference.module 
  name: (identifier) @name.reference.identifier) @reference.identifier

; ENHANCED FUNCTION DETECTION PATTERNS

; Operator overloads
(function_definition
  declarator: (function_declarator
    declarator: (operator_name) @name.definition.operator)) @definition.operator

; Destructor definitions  
(function_definition
  declarator: (function_declarator
    declarator: (destructor_name) @name.definition.destructor)) @definition.destructor

; Additional function declarations in headers
(declaration
  declarator: (function_declarator
    declarator: (identifier) @name.definition.function)) @definition.function

; Method declarations in class bodies
(field_declaration
  declarator: (function_declarator
    declarator: (field_identifier) @name.definition.method)) @definition.method

; Constructor declarations
(field_declaration
  declarator: (function_declarator
    declarator: (field_identifier) @name.definition.constructor)) @definition.constructor

; Static method declarations
(field_declaration
  specifiers: (storage_class_specifier)
  declarator: (function_declarator
    declarator: (field_identifier) @name.definition.method)) @definition.method

; Inline function definitions
(function_definition
  specifiers: (storage_class_specifier)
  declarator: (function_declarator
    declarator: (identifier) @name.definition.function)) @definition.function

; Virtual method declarations
(field_declaration
  specifiers: (virtual_specifier)
  declarator: (function_declarator
    declarator: (field_identifier) @name.definition.method)) @definition.method

; Pure virtual method declarations
(field_declaration
  specifiers: (virtual_specifier)
  declarator: (function_declarator
    declarator: (field_identifier) @name.definition.method)
  default_value: (number_literal)) @definition.method

; Friend function declarations
(friend_declaration
  (function_definition
    declarator: (function_declarator
      declarator: (identifier) @name.definition.function))) @definition.function

; Friend function declarations (declaration form)
(friend_declaration
  (declaration
    declarator: (function_declarator
      declarator: (identifier) @name.definition.function))) @definition.function