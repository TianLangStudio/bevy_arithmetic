use rand::{thread_rng, Rng};

pub struct Game {
    level: i32,
    is_pause: bool,
    answer: i32,
    question: String,
    options:[i32;12],
    scores: i32,
    need_update: bool,
}

impl Game {
      pub fn new() -> Self {
          Game{

              level:1,
              is_pause:false,
              answer: 2,
              question: "1+1=?".to_string(),
              options: [1,2,3,4,5,6,7,8,9,10,11,12],
              scores: 0,
              need_update: false,
          }
      }
      pub fn question(&self) -> String {
          self.question.clone()
      }
      pub fn options(&self) -> [i32;12] {
          self.options
      }
      pub fn scores(&self) -> i32 {
          self.scores
      }
      pub fn up_level(&mut self) {
          self.level = self.level + 1;
      }
      pub fn current_level(&self) -> i32 {
          self.level
      }
      pub fn is_pause(&self) -> bool {
          self.is_pause
      }
      pub fn is_need_update(&self) -> bool {
          self.need_update
      }
      pub fn update_complete(&mut self) {
          self.need_update = false;
      }
      pub fn answer(&mut self, option: String) -> bool{
          println!("option:{},answer:{}", option, self.answer);
        if option == self.answer.to_string() {
            println!("right");
            self.answer_right();
            true
        }else {
            println!("error");
            self.answer_error();
            false
        }
      }
      fn answer_right(&mut self){
          //加分
          self.scores += 1;
          //重新生成问题
          if self.scores > 18 {
              self.level += 1;
              self.scores = 0;
          }
          self.gen_question();
          self.need_update = true;
      }

      fn answer_error(&mut self) {
          self.scores -= 1;
          self.need_update = true;
      }
      fn gen_option(&mut self) {
          let mut rng = thread_rng();
          let options = &mut self.options;
          //rng.fill(&mut self.options);
          let len = options.len();
          for i in 0..len {
              options[i] = rng.gen_range(self.answer/2..self.answer*2);
          }
          let index: usize = rng.gen_range(0..len);
          self.options[index] = self.answer;
      }
      fn gen_question(&mut self) {
          let total = 10 * self.level;
          let mut rng = thread_rng();
          self.answer = rng.gen_range(total/2..=total);
          let n: i32 = rng.gen_range(0..=total/2);
          let m: i32 = self.answer - n;
          self.question = format!("{:?}+{:?}=?",n,m);
          self.gen_option();
      }
}