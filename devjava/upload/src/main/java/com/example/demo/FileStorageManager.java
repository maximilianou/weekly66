package com.example.demo;

import java.time.LocalDateTime;
import java.util.Random;

import org.springframework.scheduling.annotation.Async;
import org.springframework.stereotype.Service;
import org.springframework.web.multipart.MultipartFile;


@Service
class FileStorageManager {

  @Async
  public void save(MultipartFile file) throws InterruptedException {

    Thread.sleep(new Random().nextLong(4000, 8000));
    System.out.println(file.getOriginalFilename() + " is uploaded at " + LocalDateTime.now());
  }
}