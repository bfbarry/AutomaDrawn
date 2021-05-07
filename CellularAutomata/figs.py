import numpy as np
import matplotlib.pyplot as plt
import matplotlib.image as mpimg
from matplotlib import colors

### Plotting figures for notebook explanation ###

def possible_neighbors():
    plt.figure(figsize=(12,4))
    for ix,i in enumerate(('111','110','101','100','011','010','001','000')):

        plt.subplot(2,4,ix+1)
        plt.cool()
        plt.imshow([[int(p) for p in i]],vmin=0,vmax=1)
    #     plt.yticks([])
    #     plt.xticks([])
        plt.axis('off')
        plt.title('    '.join([p for p in i]),size=32)
    plt.tight_layout()
    
def two_steps():
    for ix, i in enumerate(([0,0,0,0,0,0,1,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,1,0,0,0,0,0,0])):
        plt.subplot(2,1,ix+1)
        plt.cool()
        plt.imshow([i],vmin=0,vmax=1)
        plt.axis('off')
    plt.tight_layout()