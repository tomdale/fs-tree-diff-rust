"use strict";

var FSTreeDiff = require('../native');
var FSTree = FSTreeDiff.FSTree;
var fromPaths = FSTreeDiff.fromPaths;

Object.defineProperty(FSTree.prototype, 'size', {
  configurable: false,
  get: function() {
    return this.get('size');
  }
});

FSTree.fromPaths = function(paths, options) {
  return fromPaths(paths);
};

module.exports = FSTree;
