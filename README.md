//Задaние по Highload Dev
src/main.rs - код прокси сервера
ClientEmulation.py - эмуляция запросов от пользователей, требуется модуль mechanize (pip install mechanize)
ServerEmulation/startwsgi.sh- эмуляция сервера с задержкой 2 секунды, требуются flask и uwsgi (pip install flask uwsgi)

Так же во вкладке релизы выложенны исполняемые файлы для Windows и Linux.
Запускается командой ./web_proxy_x64 -U <URL вебсайта, http://127.0.0.1:8000 по умолчанию> -B <хост:порт для подключений клиентов, 127.0.0.1:8888 по умолчанию>
