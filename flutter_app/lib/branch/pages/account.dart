import 'package:flutter/material.dart';

class AccountPage extends StatelessWidget {
  static Route<dynamic> route() => MaterialPageRoute(
        builder: (context) => AccountPage(),
      );

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Account"),
      ),
      body: Container(
          child: Column(
        children: [
          Container(
            padding: EdgeInsets.fromLTRB(10, 10, 10, 0),
            height: 200,
            width: double.maxFinite,
            child: Card(
              elevation: 2.5,
            ),
          ),
          Container(
            padding: EdgeInsets.fromLTRB(10, 10, 10, 0),
            height: 500,
            width: double.maxFinite,
            child: Card(
              elevation: 2.5,
              child: Container(
                child: Text("Hello"),
              ),
            ),
          ),
        ],
      )),
    );
  }
}
