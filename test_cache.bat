@echo off
echo Testing LiveSessions cache functionality...
echo.

echo First request (should call API):
curl -s "http://localhost:2992/gift/live_sessions?room_id=21696950&month=202604" > nul
echo.

timeout /t 2 /nobreak > nul

echo Second request (should hit cache):
curl -s "http://localhost:2992/gift/live_sessions?room_id=21696950&month=202604" > nul
echo.

echo Test completed. Check server logs above for cache behavior.