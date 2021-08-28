export const hash = (content: string): string => {
  let hashRes = 0;
  let i;
  let chr;
  if (content.length === 0) {
    return '';
  }
  for (i = 0; i < content.length; i++) {
    chr = content.charCodeAt(i);
    // tslint:disable-next-line:no-bitwise
    hashRes = ((hashRes << 5) - hashRes) + chr;
    // tslint:disable-next-line:no-bitwise
    hashRes |= 0; // Convert to 32bit integer
  }

  return hashRes.toString(16);
};

/**
 * generate unique data attribute by content of caption
 */
export const genHashDataAttr = (content: string): string => {
  const hashRes = hash(content);
  return `data-md-${hashRes}`;
};

/**
 * custom render for marked to add unique data attribute for every h1, h2 ... element
 */
export const renderer = {
  heading(text: string, level: number): string {

    return `<h${level} ${genHashDataAttr(text)}>${text}</h${level}>`;
  }
};
