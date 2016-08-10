var Benchmark = require('benchmark');
var JSFSTree = require('fs-tree-diff');
var RustFSTree = require('.');
var leftPad = require('left-pad');

var suite = new Benchmark.Suite();

var data = [];

for (var i = 0; i < 1000000; i++) {
  var index = leftPad(i + 1, 7, 0);
  data.push('bar' + index + '/');
  data.push('bar' + index + '/baz.js');
}

for (var i = 0; i < 1000000; i++) {
  var index = leftPad(i + 1, 7, 0);
  data.push('foo' + index + '.js');
}

var dataJoined = data.join('\n');

suite
.add('Rust', function() {
  JSON.parse('[' + RustFSTree.calculatePatchFromPaths('', dataJoined) + ']');
  // var fsTree = new RustFSTree();

  // fsTree.calculatePatch(RustFSTree.fromPaths(/*[
  //   'bar/',
  //   'bar/baz.js',
  //   'foo.js',
  //   ]*/ data));
})
.add('JavaScript', function() {
  var fsTree = new JSFSTree();

  fsTree.calculatePatch(JSFSTree.fromPaths(/*[
    'bar/',
    'bar/baz.js',
    'foo.js',
  ]*/ data));
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
