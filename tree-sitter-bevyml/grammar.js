/**
 * @file A custom bevy markup language for building UI's using HTML+CSS
 * @author Armand Burger
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
//// @ts-check

export default grammar({
  name: "bevyml",

  extras: $ => [
    /\s+/,
  ],

  rules: {
    document: $ => repeat($._node),

    _node: $ => choice(
      $.doctype,
      $.element,
      $.entity,
      $.plain_ampersand,
      $.text,
    ),

    doctype: $ => token(seq(
      '<!',
      /DOCTYPE/i,
      optional(seq(/\s+/, /[^>]+/)),
      '>',
    )),

    element: $ => choice(
      $.self_closing_element,
      seq(
        $.start_tag,
        repeat($._node),
        $.end_tag,
      ),
    ),

    start_tag: $ => seq(
      '<',
      optional($._tag_whitespace),
      $.tag_name,
      repeat(seq($._tag_whitespace, $.attribute)),
      optional($._tag_whitespace),
      '>',
    ),

    end_tag: $ => seq(
      '</',
      optional($._tag_whitespace),
      $.tag_name,
      optional($._tag_whitespace),
      '>',
    ),

    self_closing_element: $ => seq(
      '<',
      optional($._tag_whitespace),
      $.tag_name,
      repeat(seq($._tag_whitespace, $.attribute)),
      optional($._tag_whitespace),
      '/',
      optional($._tag_whitespace),
      '>',
    ),

    attribute: $ => seq(
      $.attribute_name,
      optional(seq(
        '=',
        $.attribute_value,
      )),
    ),

    attribute_value: $ => choice(
      $.quoted_attribute_value,
      $.unquoted_attribute_value,
    ),

    quoted_attribute_value: $ => token(choice(
      seq('"', repeat(/[^"]*/), '"'),
      seq("'", repeat(/[^']*/), "'"),
    )),

    unquoted_attribute_value: _ => token(/[^\s"'=<>`]+/),

    tag_name: _ => /[A-Za-z][A-Za-z0-9:._-]*/,

    attribute_name: _ => /[^\s"'=<>\/]+/,

    _tag_whitespace: _ => token(/\s+/),

    entity: _ => /&(#([xX][0-9a-fA-F]{1,6}|[0-9]{1,5})|[A-Za-z]{1,30});?/,

    plain_ampersand: _ => '&',

    text: _ => token(/[^<&]+/),

  },
});
