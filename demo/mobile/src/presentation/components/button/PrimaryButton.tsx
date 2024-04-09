import {styled} from 'nativewind';
import React from 'react';
import {GestureResponderEvent, TouchableOpacity} from 'react-native';
import TextPrimary from '../text/TextPrimary';

const StyledTouchableOpacity = styled(TouchableOpacity);

interface PrimaryButtonProps {
  label?: string;
  classStyle?: string;
  child?: React.ReactElement<any, any>;
  onPress: (event: GestureResponderEvent) => void;
}

const PrimaryButton: React.FC<PrimaryButtonProps> = ({
  label,
  onPress,
  child,
  classStyle,
}) => {
  return (
    <StyledTouchableOpacity
      onPress={onPress}
      className={'rounded-2xl bg-primary-normal w-full p-5 ' + classStyle}>
      {label ? (
        <TextPrimary
          classStyle="text-center text-white"
          text={label}
          isBold={true}
        />
      ) : (
        child
      )}
    </StyledTouchableOpacity>
  );
};

export default PrimaryButton;
