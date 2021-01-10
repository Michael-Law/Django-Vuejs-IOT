import 'package:flutter/material.dart';
import 'package:flutter_app/branch/tabs/tabs_pages.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'App',
      theme: ThemeData(
        primarySwatch: Colors.green,
        visualDensity: VisualDensity.adaptivePlatformDensity,
      ),
      home: TabsPage(),
      debugShowCheckedModeBanner: false,
    );
  }
}
