#[macro_use]
extern crate rustler;

use rustler::schedule::SchedulerFlags;
use rustler::{Encoder, Env, NifResult, Term};
use zxcvbn::{Entropy, ZxcvbnError};

mod atoms {
  rustler_atoms! {
      atom ok;
      atom error;

      atom nil;

      atom score;
      atom guesses;
      atom guesses_log10;
      atom calc_time;
      atom crack_times_display;
      atom online_throttling_100_per_hour;
      atom online_no_throttling_10_per_second;
      atom offline_slow_hashing_1e4_per_second;
      atom offline_fast_hashing_1e10_per_second;
      atom feedback;
      atom warning;
      atom suggestions;
  }
}

rustler_export_nifs! {
  "Elixir.Zxcvbn",
  [
      ("run_nif", 2, run, SchedulerFlags::DirtyCpu)
  ],
  None
}

struct EntropyWrap {
  entropy: Entropy,
}

impl Encoder for EntropyWrap {
  fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
    let entropy = &self.entropy;

    let crack_times = ::rustler::types::map::map_new(env)
      .map_put(
        atoms::offline_fast_hashing_1e10_per_second().encode(env),
        entropy
          .crack_times()
          .offline_fast_hashing_1e10_per_second()
          .to_string()
          .encode(env),
      )
      .ok()
      .unwrap()
      .map_put(
        atoms::offline_slow_hashing_1e4_per_second().encode(env),
        entropy
          .crack_times()
          .offline_slow_hashing_1e4_per_second()
          .to_string()
          .encode(env),
      )
      .ok()
      .unwrap()
      .map_put(
        atoms::online_throttling_100_per_hour().encode(env),
        entropy
          .crack_times()
          .online_throttling_100_per_hour()
          .to_string()
          .encode(env),
      )
      .ok()
      .unwrap()
      .map_put(
        atoms::online_no_throttling_10_per_second().encode(env),
        entropy
          .crack_times()
          .online_no_throttling_10_per_second()
          .to_string()
          .encode(env),
      )
      .ok()
      .unwrap();

    let mut feedback = ::rustler::types::map::map_new(env);
    feedback = match entropy.feedback().clone() {
      Some(feedback_value) => feedback
        .map_put(
          atoms::warning().encode(env),
          match feedback_value.warning() {
            Some(warning_value) => warning_value.to_string().encode(env),
            None => atoms::nil().encode(env),
          },
        )
        .ok()
        .unwrap()
        .map_put(
          atoms::suggestions().encode(env),
          match feedback_value.suggestions() {
            [] => atoms::nil().encode(env),
            vector => vector
              .iter()
              .map(|s| s.to_string())
              .collect::<Vec<_>>()
              .encode(env),
          },
        )
        .ok()
        .unwrap(),
      None => feedback
        .map_put(atoms::warning().encode(env), atoms::nil().encode(env))
        .ok()
        .unwrap()
        .map_put(atoms::suggestions().encode(env), atoms::nil().encode(env))
        .ok()
        .unwrap(),
    };

    let result = ::rustler::types::map::map_new(env)
      .map_put(atoms::score().encode(env), entropy.score().encode(env))
      .ok()
      .unwrap()
      .map_put(atoms::guesses().encode(env), entropy.guesses().encode(env))
      .ok()
      .unwrap()
      .map_put(
        atoms::guesses_log10().encode(env),
        entropy.guesses_log10().encode(env),
      )
      .ok()
      .unwrap()
      .map_put(atoms::crack_times_display().encode(env), crack_times)
      .ok()
      .unwrap()
      .map_put(
        atoms::calc_time().encode(env),
        // `calculation_time()` is a `Duration`
        // in seconds. We're expecting ms so
        // we multiply that by 1000.
        entropy
          .calculation_time()
          .mul_f32(1000.0)
          .as_secs_f32()
          .encode(env),
      )
      .ok()
      .unwrap()
      .map_put(atoms::feedback().encode(env), feedback)
      .ok()
      .unwrap();

    result
  }
}

fn run<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
  let password: &str = args[0].decode()?;
  let user_inputs = args[1]
    .into_list_iterator()?
    .map(|i| match i.decode() {
      Ok(value) => value,
      Err(_) => "",
    })
    .collect::<Vec<_>>();

  match zxcvbn::zxcvbn(password, &user_inputs) {
    Ok(entropy) => Ok((atoms::ok(), EntropyWrap { entropy }).encode(env)),
    Err(ZxcvbnError::BlankPassword) => Ok((atoms::error(), "blank_password").encode(env)),
  }
}
