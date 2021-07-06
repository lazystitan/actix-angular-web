import {
  AfterViewChecked,
  Component,
  Input,
  OnInit,
} from '@angular/core';
import {Post} from "../../model/post";
import {ActivatedRoute} from "@angular/router";
import {PostService} from "../../service/posts/post.service";
import {Location} from '@angular/common';
import marked from "marked";
import hljs from "highlight.js"

@Component({
  selector: 'app-post',
  templateUrl: './post.component.html',
  styleUrls: ['./post.component.scss']
})
export class PostComponent implements OnInit, AfterViewChecked {

  @Input() post: Post = {
    author: "",
    digest: '',
    content: "",
    create_time: "",
    id: 0,
    last_update_time: "",
    title: ""
  }

  constructor(
    private route: ActivatedRoute,
    private postService: PostService,
    private location: Location
  ) {
    console.log('post construct')
  }

  ngOnInit(): void {
    console.log('post init')
    this.getPost();
  }

  getPost(): void {
    let id;
    if (this.route.snapshot.paramMap.get('id')) {
      id = +this.route.snapshot.paramMap.get('id')!;
    } else {
      return;
    }
    this.postService.getPost(id)
      .subscribe(post => {
        this.post = post
        this.post.content =  marked(this.post.content)
      });
  }

  goBack(): void {
    this.location.back();
  }

  ngAfterViewChecked() {
    hljs.highlightAll()
  }

}
