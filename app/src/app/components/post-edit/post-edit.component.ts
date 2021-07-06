import { Component, OnInit } from '@angular/core';
import {FormBuilder, FormGroup} from "@angular/forms";
import {PostService} from "../../service/posts/post.service";
import {PostInsert} from "../../model/post";

@Component({
  selector: 'app-post-edit',
  templateUrl: './post-edit.component.html',
  styleUrls: ['./post-edit.component.scss']
})
export class PostEditComponent implements OnInit {

  postForm: FormGroup

  constructor(
    private formBuilder: FormBuilder,
    private postService: PostService
  ) {
    this.postForm = this.formBuilder.group({
      'title': "",
      'digest':"",
      'content': "",
    })
  }

  ngOnInit(): void {
  }

  onSubmit(post: PostInsert) {
    this.postService.addPost(post).subscribe(res => {
      if (res.code == 0) {
        alert('成功')
      } else {
        alert('失败')
      }
    })
  }

}
