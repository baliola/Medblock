import { useState } from "react";
import { Button, Icon, Input, InputGroup, InputProps, InputRightElement } from "@chakra-ui/react";
import { AiFillEye, AiFillEyeInvisible } from 'react-icons/ai';

export const PasswordInput = (props: InputProps) => {
  const [show, setShow] = useState(false);

  const handleClick = () => setShow(!show);

  return (
    <InputGroup size={'md'}>
      <Input
        {...props}
        type={show ? "text" : "password"}
        outline={'none'}
        py={6}
        bg={'primary.100'}
        px={{ base: 5, md: 5 }}
        pr="4.5rem"
        rounded={'lg'}
        _placeholder={{ color: 'gray.500' }}
        _hover={{ borderColor: 'gray.400' }}
      />
      <InputRightElement width='3.5rem' pe={2}>
        <Button
          variant={'ghost'}
          h='1.75rem'
          pt={2}
          color={'gray.300'}
          _hover={{
            bg: 'transparent',
            color: 'gray.500'
          }}
          onClick={handleClick}
        >
          {show
            ? <Icon as={AiFillEyeInvisible}
              boxSize={{ base: 5, md: 5 }}
            />
            : <Icon as={AiFillEye}
              boxSize={{ base: 5, md: 5 }}
            />
          }
        </Button>
      </InputRightElement>
    </InputGroup>
  )
}