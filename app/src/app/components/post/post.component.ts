import {AfterViewChecked, Component, ElementRef, Input, OnInit, Pipe, PipeTransform, ViewChild,} from '@angular/core';
import {Post} from '../../model/post';
import {ActivatedRoute} from '@angular/router';
import {PostService} from '../../service/posts/post.service';
import {Location} from '@angular/common';
import marked from 'marked';
import hljs from 'highlight.js';
import {transferHeadersToDom} from './mdCatalogueGen';
import {renderer} from './mdRender';
import {DomSanitizer} from '@angular/platform-browser';

/**
 * default innerHtml command doesn't render data attribute,
 * this is a pipe allow it.
 */
@Pipe({name: 'safeHtml'})
export class SafeHtmlPipe implements PipeTransform {
  constructor(private sanitized: DomSanitizer) {
  }

  transform(value: any): any {
    return this.sanitized.bypassSecurityTrustHtml(value);
  }
}

@Component({
  selector: 'app-post',
  templateUrl: './post.component.html',
  styleUrls: ['./post.component.scss']
})
export class PostComponent implements OnInit, AfterViewChecked {
  @ViewChild('catalogueRef') catalogueRef!: ElementRef;
  @Input() post: Post = {
    author: '',
    digest: '',
    content: '',
    create_time: '',
    id: 0,
    last_update_time: '',
    title: ''
  };

  catalogue = '';

  constructor(
    private route: ActivatedRoute,
    private postService: PostService,
    private location: Location
  ) {
    console.log('post construct');
  }

  genCatalogue(content: string): string {
    const lines = content.split(/[\r|\n]/).filter((line) => {
      const reg = /^#+/;
      return reg.test(line);
    });

    if (lines.length <= 0) {
      return '';
    }

    return transferHeadersToDom(lines);
  }

  ngOnInit(): void {
    console.log('post init');
    this.getPost();
  }

  /**
   * bind click event handler for catalogue
   */
  makeBindWithContentAndCatalogue(): void {
    setTimeout(() => {
      this.catalogueRef.nativeElement.addEventListener('click', (e: Event) => {
        /**
         * if event dispatcher is a HTMLElement,
         * and has attribute start with 'data-md',
         * then find the element with same attribute but not div element,
         * scroll to it
         */
        if (e.target instanceof HTMLElement) {
          const reg = /data-md.+/;
          const target = e.target;
          if (target.hasAttributes()) {
            const attrs = target.attributes;
            // tslint:disable-next-line:prefer-for-of
            for (let i = 0; i < attrs.length; i++) {
              if (reg.test(attrs[i].name)) {
                const el = document.querySelector(`[${attrs[i].name}]:not(div)`);
                if (el) {
                  el.scrollIntoView({behavior: 'smooth'});
                }
                break;
              }
            }

          }
        }
      });
    }, 0);
  }


  getPost(): void {
    let id;
    if (this.route.snapshot.paramMap.get('id')) {
      id = parseInt(this.route.snapshot.paramMap.get('id') ?? '0', 0);
    } else {
      return;
    }
    this.postService.getPost(id)
      .subscribe(post => {
        this.post = post;
        this.catalogue = this.genCatalogue(this.post.content);
        marked.use({renderer});
        this.post.content = marked(this.post.content);
        this.makeBindWithContentAndCatalogue();
      });
  }

  goBack(): void {
    this.location.back();
  }

  ngAfterViewChecked(): void {
    hljs.highlightAll();
  }

}
