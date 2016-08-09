var Benchmark = require('benchmark');
var JSFSTree = require('fs-tree-diff');
var RustFSTree = require('.');

var suite = new Benchmark.Suite();

suite
.add('Rust', function() {
  var fsTree = new RustFSTree();

  fsTree.calculatePatch(RustFSTree.fromPaths([
    'bar/',
    'bar/baz.js',
    'foo.js',
  ]));
})
.add('JavaScript', function() {
  var fsTree = new JSFSTree();

  fsTree.calculatePatch(JSFSTree.fromPaths([
    'bar/',
    'bar/baz.js',
    'foo.js',
  ]));
})
// add listeners
.on('cycle', function(event) {
  console.log(String(event.target));
})
.on('complete', function() {
  console.log('Fastest is ' + this.filter('fastest').map('name'));
})
// run async
.run({ 'async': true });
