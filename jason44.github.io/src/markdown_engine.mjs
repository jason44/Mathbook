import {unified} from 'unified'
//import {read} from 'to-vfile'
import remarkParse from 'remark-parse'
import remarkFrontmatter from 'remark-frontmatter'
import remarkGfm from 'remark-gfm'
import remarkRehype from 'remark-rehype'
import removeBreaks from 'remark-breaks'
import rehypeStringify from 'rehype-stringify'
import remarkMath from 'remark-math'
import rehypeMathjax from 'rehype-mathjax'
import fs from 'fs'
/*
const {unified} = require('unified')
const remarkParse = requrie('remark-parse');
const remarkFrontmatter = require('remark-frontmatter');
const remarkGfm = require('remark-rehype');
const rehypeStringify = require('rehype-stringify');
const fs = require('fs'); */

const save_path = './pages/'

function to_html(filePath, file) {
  fs.readFile(filePath, 'utf8', (err, data) => {
    if (err) throw err;
    convert_data(data, file);
  });
}

async function convert_data(data, file) {
  const parsed = await unified()
    .use(remarkParse)
    .use(remarkFrontmatter)
    .use(remarkGfm)
    .use(remarkRehype)
    .use(remarkMath)
    .use(rehypeMathjax)
    .use(rehypeStringify)
    .use(removeBreaks)
    .process(data);

  //console.log(String(parsed))
  // TODO: insert it into a div 
  fs.writeFile(save_path + file.replace('.md', '.html'), String(parsed), (err) => {
    if (err) throw err;
  });
}

/*
module.exports = {
	to_html,
}; */

var markdownEngine = {
  to_html: to_html
}

export default markdownEngine;