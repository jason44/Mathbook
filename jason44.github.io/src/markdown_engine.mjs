import { unified } from 'unified'
//import {read} from 'to-vfile'
import remarkParse from 'remark-parse'
import remarkFrontmatter from 'remark-frontmatter'
import remarkGfm from 'remark-gfm'
import remarkRehype from 'remark-rehype'
import rehypeStringify from 'rehype-stringify'
import remarkMath from 'remark-math'
import rehypeMathjax from 'rehype-mathjax'
//import remarkBreaks from 'remark-breaks'
import fs from 'fs'
import { JSDOM } from 'jsdom'
/*
const {unified} = require('unified')
const remarkParse = requrie('remark-parse');
const remarkFrontmatter = require('remark-frontmatter');
const remarkGfm = require('remark-rehype');
const rehypeStringify = require('rehype-stringify');
const fs = require('fs'); */

const save_path = './pages/'
const save_path_temp = './pages/tmp/'

function to_html(filePath, file) {
    fs.readFile(filePath, 'utf8', (err, data) => {
        if (data.length === 0) return;
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
        //.use(rehypeKatex)
        .use(rehypeStringify)
        //.use(remarkBreaks)
        .process(data);

    //console.log(String(parsed))
    // TODO: insert it into a div 
    const html_string = String(parsed);
    fs.writeFile(save_path + file.replace('.md', '.html'), html_string, (err) => {
        if (err) throw err;
    });
    insert_to_template(html_string, file);
}

// ignores emtpy files
function insert_to_template(html_data, file) {
    const template_html = fs.readFileSync('./article_template.html');
    const dom = new JSDOM(template_html);

    dom.window.document.addEventListener("DOMContentLoaded", (event) => {
        const child = new JSDOM(html_data);
        child.window.document.addEventListener("DOMContentLoaded", (event) => {
            //const child_p = child.window.document.querySelectorAll('p');
            const child_elements = child.window.document.children;
            //const child_style = child.window.document.querySelector('style');
            //const head = dom.window.document.querySelector('head');
            const article = dom.window.document.querySelector('.article');

            //head.appendChild(child_style);
            for (var i = 0; i < child_elements.length; i++) {
                article.appendChild(child_elements.item(i));
            }

            const output = dom.serialize();
            fs.writeFileSync(save_path_temp + file.replace('.md', '.html'), output, (err) => {
                if (err) throw err;
            });
        });
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