import React from 'react';
import {styled} from 'nativewind';
import {View} from 'react-native';
import Dialog from 'react-native-dialog';
import Colors from '@constants/colors';
const StyledView = styled(View);

interface BasicModalProps {
  show: boolean;
  onClose: () => void;
  child: React.ReactElement<any, any>;
}

const BasicModal: React.FC<BasicModalProps> = ({onClose, child, show}) => {
  return (
    <Dialog.Container
      visible={show}
      onBackdropPress={onClose}
      contentStyle={{
        borderRadius: 30,
        elevation: 0,
        backgroundColor: Colors.white,
      }}
      headerStyle={{display: 'none'}}>
      <StyledView className="bg-white h-auto w-full rounded-xl flex justify-center items-center p-2">
        {child}
      </StyledView>
    </Dialog.Container>
  );
};

export default BasicModal;
