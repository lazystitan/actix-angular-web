use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, Result, web};
use serde::{Deserialize, Serialize};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world! This is riton!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Serialize, Deserialize)]
struct Hero {
    id: i32,
    name: String,
}

#[get("/heroes")]
async fn heroes() -> Result<HttpResponse> {
    let heros = vec![Hero { id: 11, name: String::from("Dr Nice") },
                     Hero { id: 12, name: String::from("Narco") },
                     Hero { id: 13, name: String::from("Bombasto") },
                     Hero { id: 14, name: String::from("Celeritas") },
                     Hero { id: 15, name: String::from("Magneta") },
                     Hero { id: 16, name: String::from("RubberMan") },
                     Hero { id: 17, name: String::from("Dynama") },
                     Hero { id: 18, name: String::from("Dr IQ") },
                     Hero { id: 19, name: String::from("Magma") },
                     Hero { id: 20, name: String::from("Tornado") }];

    Ok(HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .json(heros))
}

#[derive(Serialize, Deserialize)]
struct Post {
    id: i32,
    author: String,
    content: String,
    create_time: String,
    last_update_time: String,
    title: String
}

#[get("/posts")]
async fn posts() -> Result<HttpResponse> {
    let posts = vec![Post {
                         id: 0,
                         title: String::from("没有意义"),
                         author: String::from("riton"),
                         content: String::from("我不能决定我们是否存在于这个世界。

                         在有能力决定之前，我已经存在于此。与此同时，我也面临必将到来的死亡。

                         我是一个唯物主义无神论者，所以我还知道并不得不接受，死亡，是最终的结束。

                         在死亡之后，一切不可感知，一切无需感知，世界从那一刻起，它与它之内发生的任何事情，都与我没有任何联系——因为我已经死了。

                         于是我获得了第一个基础：我的存在和死亡，都是不能选择的。

                         那么，我存在于此的意义是什么？有没有什么目标、责任、事情是我先天被赋予的，是我应该去做的？我在这短暂又漫长的一生中，所做的事情，有什么意义？

                         什么是意义？我问自己。

                         工作，然后我就可以父母年老之时，赡养父母；结婚之后，抚养我的孩子；同时，我又可以充实我的精神，扩展我的兴趣，这些可以给我带来快乐。

                         看起来这就是意义——事情的意义就是另一个事情。

                         可是另一个事情的意义是什么？

                         为什么，我需要快乐？快乐的意义是什么？

                         为什么我要结婚？为什么我要有一个孩子？为什么我要赡养我的父母？赡养我的父母的意义是什么？

                         如果结婚为了有孩子，如果孩子意味着血脉的延续，那延续血脉有什么意义？

                         谈到赡养父母，问题看起来更大。我从未要求过，也没有能力选择我的出生。为什么我要为一件我没有选择的事情以及它所带来的所有后果去感恩我的父母？如果我的觉得开心的事情都可以归于我的父母，那是否所有不开心的事情是否也可以归于他们？我要因此而憎恨他们甚至讨要补偿吗？赡养父母的意义又是什么？

                         意义是很重要的东西。如果一件事情没有意义，我根本不可能去做。

                         然而如果一个事情的意义就是另一个事情，并且不允许出现循环的话——不然就可以简单的讲A的意义是B，B的意义是A，无论这个圈多长，这种说法都非常的可笑——询问意义可以无穷无尽的循环往复下去。我不能接受这种想法，不能接受它无穷的持续下去，这个想法和循环一样可笑。

                         我的理性告诉我了答案的，它其实一直在那里。

                         这条看起来要么是无穷的、要么是循环的意义的锁链，其实一开始就不存在。

                         是的，答案就是，所有的事情——如果承认所有的事情应该有意义，并且意义是一个事情所为了的另一个事情的话——所有的事情，都没有意义。

                         人生，从出生到死亡，数十年的时间里，它和它之内的，都没有意义。"),
                         create_time: String::from("2021-01-01 11:00:00"),
                         last_update_time: String::from("2021-01-01 11:00:00")
                     },
                     Post {
                         id: 1,
                         title: String::from("感性的飞跃"),
                         author: String::from("riton"),
                         content: String::from("客观与主观的分离，致使所有的主观都是感性的飞跃。

                         感性来自于两个方面：

                         1. 来自于数十亿年的进化形成的心理机制。
                         2. 来自于时代的文化。

                         但是，尽管不得不依靠感性的飞跃来得到理性的基础，但大规模的使用感性的飞跃做为不想接受的观点的驳斥是否恰当？这是否是一种滥用？"),
                         create_time: String::from("2021-01-01 11:00:00"),
                         last_update_time: String::from("2021-01-01 11:00:00")
                     },
                     Post {
                         id: 2,
                         title: String::from("选择"),
                         author: String::from("riton"),
                         content: String::from("连接理性的是感性的飞跃。在虚空中我总要接受和相信，我选择了自由意志。我有自由意志，我感觉是如此。

                         所以，选择可以分为两种，我自由意志做出的选择（我的选择），不是我自由意志做出的选择（非我的选择）。

                         选择伴随结果。在理想的情况下，我只需要承担我的选择的结果，而不需要承担非我的选择的结果——尽管事实往往并非如此。但事实如此，也不代表应然如此；应然如此，事实往往也并非如此。这只是我的伦理学，我应该去承担我的选择的结果，我不应该去承担非我的选择的结果。

                         但现实没有这么美好，我冲向许许多多无法选择的后果，这些后果也不可避免的撞向我。

                         我该如何面对它们？"),
                         create_time: String::from("2021-01-01 11:00:00"),
                         last_update_time: String::from("2021-01-01 11:00:00")
                     },
                     Post {
                         id: 3,
                         title: String::from("编写模板"),
                         author: String::from("angular"),
                         content: String::from("从 HeroesComponent 模板的底部把表示英雄详情的 HTML 代码剪切粘贴到所生成的 HeroDetailComponent 模板中。

                         所粘贴的 HTML 引用了 selectedHero。 新的 HeroDetailComponent 可以展示任意英雄，而不仅仅所选的。因此还要把模板中的所有 selectedHero 替换为 hero。"),
                         create_time: String::from("2021-01-01 11:00:00"),
                         last_update_time: String::from("2021-01-01 11:00:00")
                     },
                     Post {
                         id: 4,
                         title: String::from("自由"),
                         author: String::from("riton"),
                         content: String::from("可是如果我真的有必须要做的事情，不得不尽的责任，向我的存在和我的死亡一样，我有选择过吗？

                         如果我选择过，我一定会知道它是什么，并且我还会知道它的意义是什么；那么结果就是很明了的了，如果它真的存在的话，我一定没有能力选择。

                         如果我没有能力选择，我就被束缚了，我失去了另一个我不想失去的东西——自由。"),
                         create_time: String::from("2021-01-01 11:00:00"),
                         last_update_time: String::from("2021-01-01 11:00:00")
                     },
                     Post {
                         id: 5,
                         title: String::from("资本"),
                         author: String::from("riton"),
                         content: String::from("充分竞争的自由市场经济且人力资源供大于求的条件下，绝大多数的个体没有议价的能力，相反，其所能获得的报酬是被资本决定的。作为资本自我增值意志的贯彻者，尽责的人力资源工作者会充分考虑对象的价值和潜在风险。因为，升高的人力成本会降低竞争力，致使资本自我增值的减缓。这些风险包括被广泛认为降低产出的怀孕、直系亲属患病、对象本身患病、琐事繁多等。这些风险根据分析者的主观意识，将会有不同的权重，最终降低对象的估值。表现为男女同工不同酬，年龄升高难以找到工作等。这里包含几个隐含（或）非必要前提：第一，人力资源工作者尽责且只对资本的增值尽责；第二，没有组织起来的议价集体；第三，延长工作时间的确可以提高产出。

                         如果可能的话，我一个“人”都不想出现。被定价意味着身为人的特殊性的完全剥离——在资本面前，除了潜在产出，人什么也不是。"),
                         create_time: String::from("2021-01-01 11:00:00"),
                         last_update_time: String::from("2021-01-01 11:00:00")
                     }];
    Ok(HttpResponse::Ok()
        .header("Access-Control-Allow-Origin","*")
        .json(posts)
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            // .service(heroes)
            .route("/hey", web::get().to(manual_hello))
            .service(posts)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}