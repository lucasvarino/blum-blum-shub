const TestSuit = require('nist-randomness-test-suite');

const alpha = 0.001;

const testSuite = new TestSuit(alpha);

// you can also pass in a PRNG/RNG and the testsuite would generate 10^5 bits to test
const generator = () => Math.round(Math.random());
testSuite.frequencyTest(generator);
testSuite.runsTest(generator);
testSuite.binaryMatrixRankTest(generator);
testSuite.nonOverlappingTemplateMatchingTest(generator);

// you can pass in a bit string to test its randomness
const bitString = "1000111101";

testSuite.frequencyTest(bitString);
testSuite.runsTest(bitString);
testSuite.binaryMatrixRankTest(bitString);
testSuite.nonOverlappingTemplateMatchingTest(bitString);
