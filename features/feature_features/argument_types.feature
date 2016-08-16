Feature: Step argument type inference

  Background:
    Given a project if I don't already have one
    And the steps
      """
        Given!(c, "^a step with optional arg:( optional)?$", |_, _, (b,): (bool,)| {
          assert!(b);
        });

        Given!(c, "^a step with number arg: (\\d+)$", |_, _, (u,): (u32,)| {
          assert_eq!(u, 10);
        });

        Given!(c, "^a step with string arg: \"(.*)\"$", |_, _, (s,): (String,)| {
          assert_eq!(s, "test".to_owned());
        });

        Given!(c, "^a step with docstring arg:$", |_, _, (s,): (String,)| {
          assert_eq!(s, "docstring".to_owned());
        });

        Given!(c, "^a step with table arg:$", |_, _, (t,): (Vec<Vec<String>>,)| {
          assert_eq!(t, vec![vec!["1".to_owned(), "2".to_owned(), "3".to_owned()]]);
        });

        Given!(c, "^a step with multiple args:( optional)? (\\d+)$", |_, _, (b, u, t): (bool, u32, Vec<Vec<String>>)| {
          assert_eq!(b, true);
          assert_eq!(u, 20);
          assert_eq!(t, vec![vec!["1".to_owned()], vec!["2".to_owned()], vec!["3".to_owned()]]);
        });
      """
    Then the project compiles

  Scenario: Evaluating steps with args
    When the following feature is executed
      """
        Feature: I do a thing
          Scenario: The thing works
            Given a step with optional arg: optional
            Given a step with number arg: 10
            Given a step with string arg: "test"
            Given a step with docstring arg:
              \"\"\"
              docstring
              \"\"\"
            Given a step with table arg:
              | 1 | 2 | 3 |
            Given a step with multiple args: optional 20
              | 1 |
              | 2 |
              | 3 |
      """
    Then the feature passes with no undefined steps

