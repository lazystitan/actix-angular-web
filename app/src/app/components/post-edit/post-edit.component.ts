import { Component, OnInit } from '@angular/core';
import {FormBuilder, FormGroup} from "@angular/forms";
import {PostService} from "../../service/posts/post.service";
import {PostInsert} from "../../model/post";
import {Router} from "@angular/router";
import {MatSnackBar} from "@angular/material/snack-bar";

@Component({
  selector: 'app-post-edit',
  templateUrl: './post-edit.component.html',
  styleUrls: ['./post-edit.component.scss']
})
export class PostEditComponent implements OnInit {

  postForm: FormGroup

  constructor(
    private formBuilder: FormBuilder,
    private postService: PostService,
    private router: Router,
    private snackBar: MatSnackBar
  ) {
    this.postForm = this.formBuilder.group({
      'title': "",
      'digest':"",
      'content': "",
    });
  }

  ngOnInit(): void {
  }

  onSubmit(post: PostInsert) {
    this.postService.addPost(post).subscribe(res => {
      if (res.code == 0) {
        this.snackBar.open('成功');
        setTimeout(() => {
          this.router.navigate([`/post/${res.id}`]);
        }, 1000)
      } else {
        this.snackBar.open('失败');
      }
    })
  }

}
