import 'dart:convert';

import 'package:flutter/services.dart';

const String _CONfIG_FILE_PATH = 'assets/config.json';

Future<Map<String, dynamic>> loadConfigFile() async {
  String json = await rootBundle.loadString(_CONfIG_FILE_PATH);
  jsonDecode(json) as Map<String, dynamic>;
}
