import Head from 'next/head';
import Link from 'next/link';
import Image from 'next/image';

export default function FirstPost() {
// <Head> is a react component
  return (
    <>
      <Head>
        <title>First Post</title>
      </Head>
      <h1>First Post</h1>
      <h2>
        <Link href="/">← Back to home</Link>
      </h2>
    </>
  );
}

// images are lazily loaded. they are only loaded when they come into viewport
