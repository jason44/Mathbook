/*var http = require('http');
var fs = require('fs');
var mime = require('mime-types');
const markdownEngine = require('./markdown_engine.mjs');*/
import http from 'http'
import fs from 'fs'
import path from 'path';
import mime from 'mime-types'
import markdownEngine from './markdown_engine.mjs'


const hostname = '127.0.0.1';
const port = 3000;

const vault = '../vaults/Mathematics/';

const server = http.createServer(function (req, res) {
  res.statusCode = 200;
  res.setHeader('Content-Type', 'text/plain');
  res.end('res sent');
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
  searchDir(vault);
});

function searchDir(dir) {
  fs.readdir(dir, (err, files) => {
    if (err) throw err;
    files.forEach((file) => {
      const filePath = path.join(dir, file);
      fs.stat(filePath, (err, stats) => {
        if (err) throw err;
        if (stats.isDirectory()) searchDir(filePath);
        else {
          const mimeType = mime.lookup(filePath);
          if (mimeType == 'text/markdown') markdownEngine.to_html(filePath, file)
        }
      });
    });
    //console.log(files);
  });
}