* **[Introduce]**
    * **[Grammar]**
    * *The code structure is inspired by C++.*
    * *The command naming convention is inspired by Python.*
    * *The configuration system is inspired by Rust.*
    * *For example:*

    * **main.vnc**
        ```python
        import math
        
        str var_name = "test";
        
        def run(str param1="hello world", ...) {
            print(param1);
        }
        
        @ the main function is auto-run
        def main() {
            text = math.add(1, 5).str();
            run(text);
        }
        ```
          
    * **math.vnc**
        ```python 
        def add(int i1, int i2) {
            return i1 + i2;
        }
        ```

    * **.cfg**
        ```ini 
        [version]
        name = "name_project"
        version = "0.0.1"

        [config]
        Fvars = 16
        Fdefs = 8
        Flibs = 8

        [library]
        name_lib = "version"
        ```
    
    * **[symbol]**
    * `@    | to note`
    * `&    | to take data location`
    * `#    | to declare commands taken from an external library`
    * `*    | to take value from data location`
    * `,    | to recognize param`
    * `;    | to end the code`
    * `:    | to take the calue of area from a list and array`
    * `.    | to use function and method in class`
    * `::   | to use function and method in struct`
    * 