trait SubClass<T> {
    fn superclass(&mut self) -> &mut T;
}

macro_rules! class{  
   ($structname:ident:$classname:ident {$($name:ident:$tp:ty),*})=>(
        struct $structname {
            $( $name : $tp , )*
        }
        trait $classname {
            $( fn $name ( &mut self ) -> &mut $tp; )*
        }
        impl<T:SubClass<$structname>> $classname for T {
            $( 
                fn $name ( &mut self ) -> &mut $tp {
                    self . superclass() . $name()
                } 
            )*
        }
        impl $classname for $structname {
            $( 
                fn $name ( &mut self ) -> &mut $tp {
                    &mut self . $name
                } 
            )*
        }
   );
   ($structname:ident:$classname:ident extends $fatherstruct:ident:$fatherclass:ident {$($name:ident:$tp:ty),*})=>(
        struct $structname {
            $( $name : $tp , )*
            superobject: $fatherstruct ,
        }
        trait $classname:$fatherclass {
            $( fn $name ( &mut self ) -> &mut $tp; )*
        }
        impl SubClass<$fatherstruct> for $structname{
            fn superclass(&mut self)->&mut $fatherstruct{
                &mut (self.superobject)
            }
        }
        impl $classname for $structname {
            $( 
                fn $name ( &mut self ) -> &mut $tp {
                    &mut self . $name
                } 
trait SubClass<T> {
    fn superclass(&mut self) -> &mut T;
}

macro_rules! class{  
   ($structname:ident:$classname:ident {$($name:ident:$tp:ty),*})=>(
        struct $structname {
            $( $name : $tp , )*
        }
        trait $classname {
            $( fn $name ( &mut self ) -> &mut $tp; )*
        }
        impl<T:SubClass<$structname>> $classname for T {
            $( 
                fn $name ( &mut self ) -> &mut $tp {
                    self . superclass() . $name()
                } 
            )*
        }
        impl $classname for $structname {
            $( 
                fn $name ( &mut self ) -> &mut $tp {
                    &mut self . $name
                } 
            )*
        }
   );
   ($structname:ident:$classname:ident extends $fatherstruct:ident:$fatherclass:ident {$($name:ident:$tp:ty),*})=>(
        struct $structname {
            $( $name : $tp , )*
            superobject: $fatherstruct ,
        }
        trait $classname:$fatherclass {
            $( fn $name ( &mut self ) -> &mut $tp; )*
        }
        impl SubClass<$fatherstruct> for $structname{
            fn superclass(&mut self)->&mut $fatherstruct{
                &mut (self.superobject)
            }
        }
        impl $classname for $structname {
            $( 
                fn $name ( &mut self ) -> &mut $tp {
                    &mut self . $name
                } 
            )*
        }
   )
}
class!{
    Foo:FooTrait {
        name:i32
    }
}

class!{
    Bar:BarTrait extends Foo:FooTrait {
        age:i32
    }
}
fn printname(input:&mut FooTrait) {
    println!("{}", input.name());
}
fn main() {
    let mut bar = Bar {
        age: 14,
        superobject: Foo { name: 234 },
    };
    printname(&mut bar);
    {
        let x= bar.name();
        *x+=1;
    }
    printname(&mut bar);
}
