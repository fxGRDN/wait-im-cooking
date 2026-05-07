import 'package:wait_im_cooking/app/app.dart';
import 'package:wait_im_cooking/bootstrap.dart';

Future<void> main() async {
  await bootstrap(() => const App());
}
