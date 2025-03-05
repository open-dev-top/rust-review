// 整个模块树都植根于名为 crate 的隐式模块下。

// 在作用域中增加 use 和路径类似于在文件系统中创建软连接（符号连接，symbolic link）
// use crate::front_of_house::hosting;

// 将 pub 和 use 合起来使用。这种技术被称为 “重导出（re-exporting）
pub use crate::front_of_house::hosting;

// 7-11
mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// 7-13
// 虽然示例 7-11 和 7-13 都完成了相同的任务，但示例 7-11 是使用 use 将函数引入作用域的习惯用法。要想使用 use 将函数的父模块引入作用域，我们必须在调用函数时指定父模块，这样可以清晰地表明函数不是在本地定义的，同时使完整路径的重复度最小化。
use crate::front_of_house::hosting::add_to_waitlist;

/// 绝对路径（absolute path）是以 crate 根（root）开头的全路径；对于外部 crate 的代码，是以 crate 名开头的绝对路径，对于当前 crate 的代码，则以字面值 crate 开头。
/// 相对路径（relative path）从当前模块开始，以 self、super 或定义在当前模块中的标识符开头。
///
/// 选择使用相对路径还是绝对路径，要取决于你的项目，也取决于你是更倾向于将项的定义代码与使用该项的代码分开来移动，还是一起移动。
/// 举一个例子，如果我们要将 front_of_house 模块和 eat_at_restaurant 函数一起移动到一个名为 customer_experience 的模块中，我们需要更新 add_to_waitlist 的绝对路径，但是相对路径还是可用的。
/// 然而，如果我们要将 eat_at_restaurant 函数单独移到一个名为 dining 的模块中，还是可以使用原本的绝对路径来调用 add_to_waitlist，但是相对路径必须要更新。我们更倾向于使用绝对路径，因为把代码定义和项调用各自独立地移动是更常见的。
///
pub fn eat_at_restaurant() {
    add_to_waitlist();

    // 在绝对路径，我们从 crate 也就是 crate 根开始。crate 根中定义了 front_of_house 模块。虽然 front_of_house 模块不是公有的，不过因为 eat_at_restaurant 函数与 front_of_house 定义于同一模块中（即，eat_at_restaurant 和 front_of_house 是兄弟），我们可以从 eat_at_restaurant 中引用 front_of_house。接下来是使用 pub 标记的 hosting 模块。我们可以访问 hosting 的父模块，所以可以访问 hosting。最后，add_to_waitlist 函数被标记为 pub ，我们可以访问其父模块，所以这个函数调用是有效的！
    crate::front_of_house::hosting::add_to_waitlist();

    // 在相对路径，其逻辑与绝对路径相同，除了第一步：不同于从 crate 根开始，路径从 front_of_house 开始。front_of_house 模块与 eat_at_restaurant 定义于同一模块，所以从 eat_at_restaurant 中开始定义的该模块相对路径是有效的。接下来因为 hosting 和 add_to_waitlist 被标记为 pub，路径其余的部分也是有效的，因此函数调用也是有效的！
    front_of_house::hosting::add_to_waitlist();

    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// 通过在路径的开头使用 super ，从父模块开始构建相对路径，而不是从当前模块或者 crate 根开始。
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // 等价于 crate::deliver_order()
    }

    fn cook_order() {}

    // 这个结构体会变成公有的，但是这个结构体的字段仍然是私有的。
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 与之相反，如果我们将枚举设为公有，则它的所有成员都将变为公有。
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
