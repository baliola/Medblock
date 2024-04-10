import Colors from '@constants/colors';
import {HEIGHT} from '@constants/dimensions';
import {styled} from 'nativewind';
import React from 'react';
import {StyleSheet, TouchableOpacity, View} from 'react-native';
import {BottomSheet} from 'react-native-btr';

const StyledView = styled(View);
const StyledTouchableOpacity = styled(TouchableOpacity);

interface BasicBottomSheetProps {
  visible: boolean;
  onClose: (() => void) | undefined;
  height?: number;
  child: React.ReactElement<any, any>;
}

const BasicBottomSheet: React.FC<BasicBottomSheetProps> = ({
  onClose,
  visible,
  height = HEIGHT * 0.3,
  child,
}) => {
  return (
    <BottomSheet
      visible={visible}
      onBackButtonPress={onClose}
      onBackdropPress={onClose}>
      <StyledView style={{...styles.card, height: height}} className="pt-4">
        <StyledTouchableOpacity
          onPress={onClose}
          className="h-2 w-14 bg-slate-500 rounded-lg"
        />
        {child}
      </StyledView>
    </BottomSheet>
  );
};

const styles = StyleSheet.create({
  card: {
    backgroundColor: Colors.white,
    alignItems: 'center',
    borderTopLeftRadius: 20,
    borderTopRightRadius: 20,
  },
});

export default BasicBottomSheet;
