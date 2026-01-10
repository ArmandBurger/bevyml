import XCTest
import SwiftTreeSitter
import TreeSitterBevyml

final class TreeSitterBevymlTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_bevyml())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading BevyML grammar")
    }
}
