; exmpale D:\yeelight-bulb-controller\yeelight-bulb-controller.exe
CONTROLLER := ""

; exmpale D:\yeelight-bulb-controller
WORKDIR := ""

; example 127.0.0.1:55443
BULB_IP := ""

; toggle the light
^!l::
    RUN, %CONTROLLER% %BULB_IP% toggle, %WORKDIR%, Hide
    return

; decrease bright
^!;::
    RUN, %CONTROLLER% %BULB_IP% set_adjust decrease bright, %WORKDIR%, Hide
    return

; increase bright
^!'::
    RUN, %CONTROLLER% %BULB_IP% set_adjust increase bright, %WORKDIR%, Hide
    return
