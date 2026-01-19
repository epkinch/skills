function test_function(callback) { return callback(); }
var text = 'Domo arigato!';
assert(test_function(function(){ return text; }) === text, "The useless function works! " + text);

// test_function is a named function, which takes a function as a parameter.
// the argument function(){ return text; } is an anonymous function.