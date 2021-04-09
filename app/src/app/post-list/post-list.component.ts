import {Component, OnInit} from '@angular/core';

import {PostService} from "../posts/post.service";
import {Post} from "../post";

@Component({
  selector: 'app-post-list',
  templateUrl: './post-list.component.html',
  styleUrls: ['./post-list.component.less']
})
export class PostListComponent implements OnInit {

  posts:Post[] = [];

  constructor(
    private postService: PostService
  ) {
  }

  ngOnInit(): void {
    this.getPosts()
  }

  getPosts():void {
    this.postService.getPosts().subscribe(posts => this.posts = posts)
  }

}
