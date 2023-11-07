package com.example.demo;


import java.util.concurrent.CompletableFuture;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.scheduling.annotation.Async;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.multipart.MultipartFile;

@Controller
public class FileAsyncUploadController {

  @Autowired
  FileStorageManager fileStorageManager;

  @Async
  @PostMapping("/uploadAsync")
  public CompletableFuture<ResponseEntity<String>> handleConcurrentFilesUpload(
      @RequestParam("files") MultipartFile[] files) throws Exception {

    // Handle empty file error
    if (files.length == 0) {
      return CompletableFuture
          .completedFuture(ResponseEntity.badRequest().body("No files submitted"));
    }
    // File upload process is submitted
    else {

      for (MultipartFile file : files) {
        fileStorageManager.save(file);
        //TODO: access and store each file into file storage
      }
      return CompletableFuture.completedFuture(
          ResponseEntity.ok("File upload started"));
    }
  }
}