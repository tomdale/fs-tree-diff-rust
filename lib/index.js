var FSTree = require('../native').FSTree;

FSTree.fromPaths = function(paths, options) {
  return new FSTree({
    entries: paths
  });
};

module.exports = FSTree;
