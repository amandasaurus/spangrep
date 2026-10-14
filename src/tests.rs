use super::*;

macro_rules! test_output {
	( $name:ident, $args:expr, $input:expr, $expected_output:expr) => {
		#[test]
		fn $name() {
			let argv = $args;
			let mut argv = argv.split(" ").collect::<Vec<_>>();
			argv.insert(0, "spangrep");
			let args = Args::try_parse_from(argv).unwrap();
			let input = ($input).as_bytes();
			let mut output = Vec::new();
			let expected_output = $expected_output;

			run(args, input, &mut output).unwrap();
			let output = String::from_utf8(output).unwrap();

			assert_eq!(
				output, expected_output,
				"Ouput was {:?} but expected {:?}",
				output, expected_output,
			);
		}
	};
}

test_output!(empty_in_out0, "", "", "");
test_output!(empty_in_out1, "", "a\nb", "a\nb\n");
test_output!(empty_in_out2, "", "a\nb", "a\nb\n");

test_output!(basic0, "-f b -t d", "a\nb\nc\nd\ne\nf", "b\nc\nd\n");
test_output!(basic1, "-f b -T d", "a\nb\nc\nd\ne\nf", "b\nc\n");
test_output!(basic2, "-F b -t d", "a\nb\nc\nd\ne\nf", "c\nd\n");
test_output!(basic3, "-F b -T d", "a\nb\nc\nd\ne\nf", "c\n");

test_output!(num_match0, "-F b -T d", "a\nb\nc\nd\ne\nf\na\nb\nc\nd\ne\nf\n", "c\nc\n");
test_output!(num_match1, "-F b -T d -m 1", "a\nb\nc\nd\ne\nf\na\nb\nc\nd\ne\nf\n", "c\n");

test_output!(only_one0, "-f b", "a\nb\nc\nd\ne\nf", "b\nc\nd\ne\nf\n");
test_output!(only_one1, "-t d", "a\nb\nc\nd\ne\nf", "a\nb\nc\nd\n");
