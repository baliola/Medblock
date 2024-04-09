import {styled} from 'nativewind';
import React from 'react';
import {GestureResponderEvent, TouchableOpacity} from 'react-native';

const StyledTouchableOpacity = styled(TouchableOpacity);

interface IconButtonProps {
  icon: React.ReactElement<any, any>;
  onPress: (event: GestureResponderEvent) => void;
  classStyle?: string;
}

const IconButton: React.FC<IconButtonProps> = ({icon, onPress, classStyle}) => {
  return (
    <StyledTouchableOpacity className={classStyle} onPress={onPress}>
      {icon}
    </StyledTouchableOpacity>
  );
};

export default IconButton;
