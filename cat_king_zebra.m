%Tech Connect MATLAB Code

%Create a GUI figure window
fig = figure;

%Render grid on the figure
grid on;

%Create a title for the figure
title("Tech Connect");

%Create x and y axis labels
xlabel("X-Axis");
ylabel("Y-Axis");

%Create a line object for the line graph
L = line(NaN, NaN, 'LineWidth', 2);

%Set the X limits of the line graph
xlim([0 30]);

%Set the Y limits of the line graph
ylim([-1 2]);

%Create a legend for the line graph
legend("Tech Connect");

%Store the data that will be plotted
x_data = [1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30];
y_data = [0 .2 .3 .5 .6 .7 .8 .7 .6 .5 .4 .3 .2 .1 0 -.1 -.2 -.3 -.2 -.1 0 .1 .2 .3 .4 .5 .4 .3 .2 .1 0];

%Create a blank plot figure
p = plot(NaN,NaN);

% Update the line graph
set(L, 'XData', x_data, 'YData', y_data);

%Update the plot figure
set(p, 'XData', x_data, 'YData', y_data);

%Set the label of the x-axis
set(p, 'XTickLabel', {'Jan','Feb','Mar','Apr','May','Jun','Jul','Aug','Sep','Oct','Nov','Dec'});

%Create a callback for the ButtonDownFcn event
set(fig, 'WindowButtonDownFcn', @clickCallback);

% Create a push button
btn = uicontrol('Style', 'pushbutton', 'String', 'Change Graph',...
    'Position', [20 20 100 20], 'Callback', @pushbtnCallback);

%Create a callback for the push button
function pushbtnCallback(~,~)
    %Vary the y data
    y_data = [0 .2 .3 .5 .6 .7 .7 .8 .7 .6 .4 .3 .2 .1 0 -.1 -.3 -.2 -.1 0 .1 .2 .3 .4 .5 .4 .3 .2 .1 0];
    
    %Update the line graph
    set(L, 'XData', x_data, 'YData', y_data);
    
    %Update the plot figure
    set(p, 'XData', x_data, 'YData', y_data);
    
    %Label the x-axis
    set(p, 'XTickLabel', {'Jan','Feb','Mar','Apr','May','Jun','Jul','Aug','Sep','Oct','Nov','Dec'});
end

%Create a callback for the click event
function clickCallback(~,~)
    %Store the current Axes
    c_axes = gca;
    
    %Obtain the coordinates of the point clicked
    cp = get(c_axes, 'CurrentPoint');
    
    %Store the x and y coordinates
    x = cp(1,1);
    y = cp(1,2);
    
    %Print the coordinates
    fprintf('Point clicked: ( %f , %f ) \n',x,y);
end